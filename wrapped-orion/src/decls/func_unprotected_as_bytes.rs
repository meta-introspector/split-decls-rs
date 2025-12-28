macro_rules! func_unprotected_as_bytes {
    () => {
        # [doc = " Macro to implement a `unprotected_as_bytes()` function for objects that"] # [doc = " implement extra protections. Typically used on objects that implement"] # [doc = " `Drop`."] macro_rules ! func_unprotected_as_bytes (() => (# [inline] # [doc = " Return the object as byte slice. __**Warning**__: Should not be used unless strictly"] # [doc = " needed. This __**breaks protections**__ that the type implements."] pub fn unprotected_as_bytes (& self) -> & [u8] { self . value [.. self . original_length] . as_ref () })) ;
    };
}

func_unprotected_as_bytes!();