// SRC: ../rust/compiler/rustc_public_bridge/src/bridge.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=10 */
// Defines a set of traits that is used for abstracting
// rustc_public's components that are needed in rustc_public_bridge.
//
// These traits are really useful when programming
// in rustc_public-agnostic settings.

use std::fmt::Debug;

use super::context::CompilerCtxt;
use super::{Bridge, Tables};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=new | COMPLEXITY=2 | LINES=5 */

pub trait Error {
    fn new(msg: String) -> Self;
    fn from_internal<T: Debug>(err: T) -> Self;
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=new | COMPLEXITY=2 | LINES=4 */

pub trait Prov<B: Bridge> {
    fn new(aid: B::AllocId) -> Self;
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=new | COMPLEXITY=2 | LINES=11 */

pub trait Allocation<B: Bridge> {
    fn new<'tcx>(
        bytes: Vec<Option<u8>>,
        ptrs: Vec<(usize, crate::rustc_middle::mir::interpret::AllocId)>,
        align: u64,
        mutability: crate::rustc_middle::mir::Mutability,
        tables: &mut Tables<'tcx, B>,
        cx: &CompilerCtxt<'tcx, B>,
    ) -> Self;
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=new | COMPLEXITY=9 | LINES=8 */

macro_rules! make_bridge_trait {
    ($name:ident) => {
        pub trait $name<B: Bridge> {
            fn new(did: B::DefId) -> Self;
        }
    };
}
/* AST_META: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=21 */

make_bridge_trait!(CrateItem);
make_bridge_trait!(AdtDef);
make_bridge_trait!(ForeignModuleDef);
make_bridge_trait!(ForeignDef);
make_bridge_trait!(FnDef);
make_bridge_trait!(ClosureDef);
make_bridge_trait!(CoroutineDef);
make_bridge_trait!(CoroutineClosureDef);
make_bridge_trait!(AliasDef);
make_bridge_trait!(ParamDef);
make_bridge_trait!(BrNamedDef);
make_bridge_trait!(TraitDef);
make_bridge_trait!(GenericDef);
make_bridge_trait!(ConstDef);
make_bridge_trait!(ImplDef);
make_bridge_trait!(RegionDef);
make_bridge_trait!(CoroutineWitnessDef);
make_bridge_trait!(AssocDef);
make_bridge_trait!(OpaqueDef);
make_bridge_trait!(StaticDef);