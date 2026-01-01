/* FP:query_context.rs-0001 */ use crate::layout;
/* FP:query_context.rs-0002 */ 
/* FP:query_context.rs-0003 */ /// Context necessary to answer the question "Are these types transmutable?".
/* FP:query_context.rs-0004 */ pub(crate) trait QueryContext {
/* FP:query_context.rs-0005 */     type Def: layout::Def;
/* FP:query_context.rs-0006 */     type Region: layout::Region;
/* FP:query_context.rs-0007 */     type Type: layout::Type;
/* FP:query_context.rs-0008 */ }
/* FP:query_context.rs-0009 */ 
/* FP:query_context.rs-0010 */ #[cfg(test)]
/* FP:query_context.rs-0011 */ pub(crate) mod test {
/* FP:query_context.rs-0012 */     use std::marker::PhantomData;
/* FP:query_context.rs-0013 */ 
/* FP:query_context.rs-0014 */     use super::QueryContext;
/* FP:query_context.rs-0015 */ 
/* FP:query_context.rs-0016 */     pub(crate) struct UltraMinimal<R = !, T = !>(PhantomData<(R, T)>);
/* FP:query_context.rs-0017 */ 
/* FP:query_context.rs-0018 */     impl<R, T> Default for UltraMinimal<R, T> {
/* FP:query_context.rs-0019 */         fn default() -> Self {
/* FP:query_context.rs-0020 */             Self(PhantomData)
/* FP:query_context.rs-0021 */         }
/* FP:query_context.rs-0022 */     }
/* FP:query_context.rs-0023 */ 
/* FP:query_context.rs-0024 */     #[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
/* FP:query_context.rs-0025 */     pub(crate) enum Def {
/* FP:query_context.rs-0026 */         HasSafetyInvariants,
/* FP:query_context.rs-0027 */         NoSafetyInvariants,
/* FP:query_context.rs-0028 */     }
/* FP:query_context.rs-0029 */ 
/* FP:query_context.rs-0030 */     impl crate::layout::Def for Def {
/* FP:query_context.rs-0031 */         fn has_safety_invariants(&self) -> bool {
/* FP:query_context.rs-0032 */             self == &Self::HasSafetyInvariants
/* FP:query_context.rs-0033 */         }
/* FP:query_context.rs-0034 */     }
/* FP:query_context.rs-0035 */ 
/* FP:query_context.rs-0036 */     impl<R, T> QueryContext for UltraMinimal<R, T>
/* FP:query_context.rs-0037 */     where
/* FP:query_context.rs-0038 */         R: crate::layout::Region,
/* FP:query_context.rs-0039 */         T: crate::layout::Type,
/* FP:query_context.rs-0040 */     {
/* FP:query_context.rs-0041 */         type Def = Def;
/* FP:query_context.rs-0042 */         type Region = R;
/* FP:query_context.rs-0043 */         type Type = T;
/* FP:query_context.rs-0044 */     }
/* FP:query_context.rs-0045 */ }
/* FP:query_context.rs-0046 */ 
/* FP:query_context.rs-0047 */ #[cfg(feature = "rustc")]
/* FP:query_context.rs-0048 */ mod rustc {
/* FP:query_context.rs-0049 */     use crate::rustc_complete::ty::{Region, Ty, TyCtxt};
/* FP:query_context.rs-0050 */ 
/* FP:query_context.rs-0051 */     use super::*;
/* FP:query_context.rs-0052 */ 
/* FP:query_context.rs-0053 */     impl<'tcx> super::QueryContext for TyCtxt<'tcx> {
/* FP:query_context.rs-0054 */         type Def = layout::rustc::Def<'tcx>;
/* FP:query_context.rs-0055 */         type Region = Region<'tcx>;
/* FP:query_context.rs-0056 */         type Type = Ty<'tcx>;
/* FP:query_context.rs-0057 */     }
/* FP:query_context.rs-0058 */ }