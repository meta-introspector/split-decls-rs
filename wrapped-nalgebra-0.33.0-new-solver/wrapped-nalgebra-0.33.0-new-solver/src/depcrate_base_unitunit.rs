// Generated macro for Unit (struct)
macro_rules! Depcrate_base_unitUnit {
() => {
// Module: crate::base::unit
// Provides: {"Unit"}
// Dependencies: {}
# [doc = " A wrapper that ensures the underlying algebraic entity has a unit norm."] # [doc = ""] # [doc = " **It is likely that the only piece of documentation that you need in this page are:**"] # [doc = " - **[The construction with normalization](#construction-with-normalization)**"] # [doc = " - **[Data extraction and construction without normalization](#data-extraction-and-construction-without-normalization)**"] # [doc = " - **[Interpolation between two unit vectors](#interpolation-between-two-unit-vectors)**"] # [doc = ""] # [doc = " All the other impl blocks you will see in this page are about [`UnitComplex`](crate::UnitComplex)"] # [doc = " and [`UnitQuaternion`](crate::UnitQuaternion); both built on top of `Unit`.  If you are interested"] # [doc = " in their documentation, read their dedicated pages directly."] # [repr (transparent)] # [derive (Clone , Hash , Copy)] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize) , archive (as = "Unit<T::Archived>" , bound (archive = "
        T: rkyv::Archive,
    ")))] # [cfg_attr (feature = "rkyv-serialize" , derive (bytecheck :: CheckBytes))] pub struct Unit < T > { pub (crate) value : T , }
};
}
