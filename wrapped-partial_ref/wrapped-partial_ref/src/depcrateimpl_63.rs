// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
# [doc = " *(internal)* To have a reference with a mutable first part as subset, pluck that part and make"] # [doc = " sure the remaining references are in a subset relation."] unsafe impl < 'a , SubsetPart , Reference , PluckedRef , PartIndex , TailIndex > HasSubset < 'a , Mut < SubsetPart , Reference > , SubsetIndexCons < PartIndex , TailIndex > > for PluckedRef where PluckedRef : PluckMut < 'a , SubsetPart , PartIndex > , < PluckedRef as PluckMut < 'a , SubsetPart , PartIndex > > :: Remainder : HasSubset < 'a , Reference , TailIndex > , Reference : HasTarget , { type Remainder = < < PluckedRef as PluckMut < 'a , SubsetPart , PartIndex > > :: Remainder as HasSubset < 'a , Reference , TailIndex , > > :: Remainder ; }
};
}
