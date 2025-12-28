macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageTlsDirectory32 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageTlsDirectory32 { pub start_address_of_raw_data : U32 < LE > , pub end_address_of_raw_data : U32 < LE > , # [doc = " PDWORD"] pub address_of_index : U32 < LE > , # [doc = " PIMAGE_TLS_CALLBACK *"] pub address_of_call_backs : U32 < LE > , pub size_of_zero_fill : U32 < LE > , pub characteristics : U32 < LE > , }
    };
}

ImageTlsDirectory32!();