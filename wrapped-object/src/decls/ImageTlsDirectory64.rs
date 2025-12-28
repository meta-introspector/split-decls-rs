macro_rules! deps {
    () => {
        U64!();
        U32!();
    };
}

macro_rules! ImageTlsDirectory64 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageTlsDirectory64 { pub start_address_of_raw_data : U64 < LE > , pub end_address_of_raw_data : U64 < LE > , # [doc = " PDWORD"] pub address_of_index : U64 < LE > , # [doc = " PIMAGE_TLS_CALLBACK *;"] pub address_of_call_backs : U64 < LE > , pub size_of_zero_fill : U32 < LE > , pub characteristics : U32 < LE > , }
    };
}

ImageTlsDirectory64!();