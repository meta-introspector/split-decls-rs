macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! FieldBytes {
    () => {
        deps!();
        # [doc = " NIST P-384 field element serialized as bytes."] # [doc = ""] # [doc = " Byte array containing a serialized field element value (base field or"] # [doc = " scalar)."] pub type FieldBytes = elliptic_curve :: FieldBytes < NistP384 > ;
    };
}

FieldBytes!()