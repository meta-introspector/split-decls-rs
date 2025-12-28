macro_rules! deps {
    () => {
        ExternalCaller!();
        Decoder!();
    };
}

macro_rules! decode {
    () => {
        deps!();
        # [doc = " Convert Punycode to Unicode."] # [doc = ""] # [doc = " Return None on malformed input or overflow."] # [doc = " Overflow can only happen on inputs that take more than"] # [doc = " 63 encoded bytes, the DNS limit on domain name labels."] pub fn decode (input : & str) -> Option < Vec < char > > { Some (Decoder :: default () . decode :: < u8 , ExternalCaller > (input . as_bytes ()) . ok () ? . collect () ,) }
    };
}

decode!();