macro_rules! deps {
    () => {
        ParamValue!();
    };
}

macro_rules! Param {
    () => {
        deps!();
        # [doc = " Provides automatic parameter conversion in cases where the Windows API expects implicit conversion support."] # [doc = ""] # [doc = " There is no need to implement this trait. Blanket implementations are provided for all applicable Windows types."] pub trait Param < T : TypeKind , C = < T as TypeKind > :: TypeKind > : Sized where T : Type < T > , { # [doc (hidden)] unsafe fn param (self) -> ParamValue < T > ; }
    };
}

Param!()