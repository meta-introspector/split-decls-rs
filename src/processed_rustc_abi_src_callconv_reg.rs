// SRC: ../rust/compiler/rustc_abi/src/callconv/reg.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
#[cfg(feature = "nightly")]
use rustc_macros::HashStable_Generic;

use crate::{Align, HasDataLayout, Size};
/* AST_META: AST_ID=2 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum RegKind {
    Integer,
    Float,
    Vector,
}
/* AST_META: AST_ID=3 | TYPE=STRUCT | NAME=Reg | COMPLEXITY=2 | LINES=7 */

#[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Reg {
    pub kind: RegKind,
    pub size: Size,
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=$name | COMPLEXITY=10 | LINES=8 */

macro_rules! reg_ctor {
    ($name:ident, $kind:ident, $bits:expr) => {
        pub fn $name() -> Reg {
            Reg { kind: RegKind::$kind, size: Size::from_bits($bits) }
        }
    };
}
/* AST_META: AST_ID=5 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

impl Reg {
    reg_ctor!(i8, Integer, 8);
    reg_ctor!(i16, Integer, 16);
    reg_ctor!(i32, Integer, 32);
    reg_ctor!(i64, Integer, 64);
    reg_ctor!(i128, Integer, 128);

    reg_ctor!(f32, Float, 32);
    reg_ctor!(f64, Float, 64);
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=align | COMPLEXITY=18 | LINES=25 */

impl Reg {
    pub fn align<C: HasDataLayout>(&self, cx: &C) -> Align {
        let dl = cx.data_layout();
        match self.kind {
            RegKind::Integer => match self.size.bits() {
                1 => dl.i1_align.abi,
                2..=8 => dl.i8_align.abi,
                9..=16 => dl.i16_align.abi,
                17..=32 => dl.i32_align.abi,
                33..=64 => dl.i64_align.abi,
                65..=128 => dl.i128_align.abi,
                _ => panic!("unsupported integer: {self:?}"),
            },
            RegKind::Float => match self.size.bits() {
                16 => dl.f16_align.abi,
                32 => dl.f32_align.abi,
                64 => dl.f64_align.abi,
                128 => dl.f128_align.abi,
                _ => panic!("unsupported float: {self:?}"),
            },
            RegKind::Vector => dl.llvmlike_vector_align(self.size).abi,
        }
    }
}