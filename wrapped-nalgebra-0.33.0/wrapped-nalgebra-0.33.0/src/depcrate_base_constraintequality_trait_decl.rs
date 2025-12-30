// Generated macro for equality_trait_decl (macro)
macro_rules! Depcrate_base_constraintequality_trait_decl {
() => {
// Module: crate::base::constraint
// Provides: {"equality_trait_decl"}
// Dependencies: {}
macro_rules ! equality_trait_decl (($ ($ doc : expr , $ Trait : ident) ,* $ (,) *) => { $ (# [doc = $ doc] pub trait $ Trait < D1 : Dim , D2 : Dim >: DimEq < D1 , D2 > + DimEq < D2 , D1 > { # [doc = " This is either equal to `D1` or `D2`, always choosing the one (if any) which is a type-level"] # [doc = " constant."] type Representative : Dim ; # [doc = " Returns a representative dimension instance if the two are equal,"] # [doc = " otherwise `None`."] fn representative (d1 : D1 , d2 : D2) -> Option << Self as $ Trait < D1 , D2 >>:: Representative > { < Self as DimEq < D1 , D2 >>:: representative (d1 , d2) . map (| common_dim | < Self as $ Trait < D1 , D2 >>:: Representative :: from_usize (common_dim . value ())) } } impl < D : Dim > $ Trait < D , D > for ShapeConstraint { type Representative = D ; } impl < D : DimName > $ Trait < D , Dyn > for ShapeConstraint { type Representative = D ; } impl < D : DimName > $ Trait < Dyn , D > for ShapeConstraint { type Representative = D ; }) * }) ;
};
}
