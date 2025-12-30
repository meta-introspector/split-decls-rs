// Generated macro for impl_17 (impl)
macro_rules! Depcrate_split_producerimpl_17 {
() => {
// Module: crate::split_producer
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'p , P , V , const INCL : bool > SplitProducer < 'p , P , V , INCL > where V : Fissile < P > + Send , { # [doc = " Common `fold_with` implementation, integrating `SplitTerminator`'s"] # [doc = " need to sometimes skip its final empty item."] pub (super) fn fold_with < F > (self , folder : F , skip_last : bool) -> F where F : Folder < V > , { let SplitProducer { data , separator , tail , } = self ; if tail == data . length () { data . fold_splits :: < F , INCL > (separator , folder , skip_last) } else if let Some (index) = data . rfind (separator , tail) { let (left , right) = data . split_once :: < INCL > (index) ; let folder = left . fold_splits :: < F , INCL > (separator , folder , false) ; if skip_last || folder . full () { folder } else { folder . consume (right) } } else { if skip_last { folder } else { folder . consume (data) } } } }
};
}
