macro_rules! deps {
    () => {
        PointerCast!();
    };
}

macro_rules! CastKind {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Clone)] pub enum CastKind { # [doc = " An exposing pointer to address cast. A cast between a pointer and an integer type, or"] # [doc = " between a function pointer and an integer type."] # [doc = " See the docs on `expose_addr` for more details."] PointerExposeAddress , # [doc = " An address-to-pointer cast that picks up an exposed provenance."] # [doc = " See the docs on `from_exposed_addr` for more details."] PointerFromExposedAddress , # [doc = " All sorts of pointer-to-pointer casts. Note that reference-to-raw-ptr casts are"] # [doc = " translated into `&raw mut/const *r`, i.e., they are not actually casts."] PtrToPtr , # [doc = " Pointer related casts that are done by coercions."] PointerCoercion (PointerCast) , # [doc = " Cast into a dyn* object."] DynStar , IntToInt , FloatToInt , FloatToFloat , IntToFloat , FnPtrToPtr , }
    };
}

CastKind!();