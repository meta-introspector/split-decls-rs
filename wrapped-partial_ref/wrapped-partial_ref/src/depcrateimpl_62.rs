// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
# [doc = " *(internal)* To have a reference with a constant first part as subset, pluck that part and make"] # [doc = " sure the remaining references are in a subset relation."] unsafe impl < 'a , SubsetPart , Reference , PluckedRef , PartIndex , TailIndex > HasSubset < 'a , Const < SubsetPart , Reference > , SubsetIndexCons < PartIndex , TailIndex > > for PluckedRef where PluckedRef : PluckConst < 'a , SubsetPart , PartIndex > , < PluckedRef as PluckConst < 'a , SubsetPart , PartIndex > > :: Remainder : HasSubset < 'a , Reference , TailIndex > , Reference : HasTarget , { type Remainder = < < PluckedRef as PluckConst < 'a , SubsetPart , PartIndex > > :: Remainder as HasSubset < 'a , Reference , TailIndex , > > :: Remainder ; }
};
}
