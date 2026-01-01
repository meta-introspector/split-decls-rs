// SRC: ../rust/compiler/rustc_public/src/target.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=MachineInfo | COMPLEXITY=2 | LINES=12 */
// Provide information about the machine that this is being compiled into.

use serde::Serialize;

use crate::compiler_interface::with;

/// The properties of the target machine being compiled into.
#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct MachineInfo {
    pub endian: Endian,
    pub pointer_width: MachineSize,
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=target | COMPLEXITY=5 | LINES=14 */

impl MachineInfo {
    pub fn target() -> MachineInfo {
        with(|cx| cx.target_info())
    }

    pub fn target_endianness() -> Endian {
        with(|cx| cx.target_info().endian)
    }

    pub fn target_pointer_width() -> MachineSize {
        with(|cx| cx.target_info().pointer_width)
    }
}
/* AST_META: AST_ID=3 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Copy, Clone, PartialEq, Eq, Serialize)]
pub enum Endian {
    Little,
    Big,
}
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=MachineSize | COMPLEXITY=2 | LINES=6 */

/// Represent the size of a component.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct MachineSize {
    num_bits: usize,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=bytes | COMPLEXITY=7 | LINES=22 */

impl MachineSize {
    #[inline(always)]
    pub fn bytes(self) -> usize {
        self.num_bits / 8
    }

    #[inline(always)]
    pub fn bits(self) -> usize {
        self.num_bits
    }

    #[inline(always)]
    pub fn from_bits(num_bits: usize) -> MachineSize {
        MachineSize { num_bits }
    }

    #[inline]
    pub fn unsigned_int_max(self) -> Option<u128> {
        (self.num_bits <= 128).then(|| u128::MAX >> (128 - self.bits()))
    }
}