// Generated macro for PointerKind (enum)
macro_rules! Depcrate_castPointerKind {
() => {
// Module: crate::cast
// Provides: {"PointerKind"}
// Dependencies: {}
# [doc = " The kind of pointer and associated metadata (thin, length or vtable) - we"] # [doc = " only allow casts between wide pointers if their metadata have the same"] # [doc = " kind."] # [derive (Debug , Copy , Clone , PartialEq , Eq , TypeVisitable , TypeFoldable)] enum PointerKind < 'tcx > { # [doc = " No metadata attached, ie pointer to sized type or foreign type"] Thin , # [doc = " A trait object"] VTable (& 'tcx ty :: List < ty :: Binder < 'tcx , ty :: ExistentialPredicate < 'tcx > > >) , # [doc = " Slice"] Length , # [doc = " The unsize info of this projection or opaque type"] OfAlias (ty :: AliasTy < 'tcx >) , # [doc = " The unsize info of this parameter"] OfParam (ty :: ParamTy) , }
};
}
