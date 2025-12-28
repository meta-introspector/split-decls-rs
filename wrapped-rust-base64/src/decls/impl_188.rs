macro_rules! deps {
    () => {
        DecodeEstimate!();
        DecodeError!();
        DecodeMetadata!();
        DecodeSliceError!();
        Engine!();
        Config!();
        DecoderReader!();
        DecoderReaderEngine!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < E : Engine > Engine for DecoderReaderEngine < E > { type Config = E :: Config ; type DecodeEstimate = E :: DecodeEstimate ; fn internal_encode (& self , input : & [u8] , output : & mut [u8]) -> usize { self . engine . internal_encode (input , output) } fn internal_decoded_len_estimate (& self , input_len : usize) -> Self :: DecodeEstimate { self . engine . internal_decoded_len_estimate (input_len) } fn internal_decode (& self , input : & [u8] , output : & mut [u8] , decode_estimate : Self :: DecodeEstimate ,) -> Result < DecodeMetadata , DecodeSliceError > { let mut reader = DecoderReader :: new (input , & self . engine) ; let mut buf = vec ! [0 ; input . len ()] ; let _ = reader . read (& mut buf) . and_then (| len | { buf . truncate (len) ; reader . read_to_end (& mut buf) }) . map_err (| io_error | { * io_error . into_inner () . and_then (| inner | inner . downcast :: < DecodeError > () . ok ()) . unwrap () }) ? ; if output . len () < buf . len () { return Err (DecodeSliceError :: OutputSliceTooSmall) ; } output [.. buf . len ()] . copy_from_slice (& buf) ; Ok (DecodeMetadata :: new (buf . len () , input . iter () . enumerate () . filter (| (_offset , byte) | * * byte == PAD_BYTE) . map (| (offset , _byte) | offset) . next () ,)) } fn config (& self) -> & Self :: Config { self . engine . config () } }
    };
}

impl_188!();