macro_rules! deps {
    () => {
        UntrustedToken!();
        Error!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < T : Purpose < V > , V : Version > TryFrom < & str > for UntrustedToken < T , V > { type Error = Error ; # [doc = " This fails if `value` is not a PASETO token or it has invalid base64 encoding."] fn try_from (value : & str) -> Result < Self , Self :: Error > { T :: validate_header (value) ? ; let parts_split = value . split ('.') . collect :: < Vec < & str > > () ; if parts_split . len () < 3 || parts_split . len () > 4 { return Err (Error :: TokenFormat) ; } if parts_split [2] . is_empty () { return Err (Error :: TokenFormat) ; } let m_raw = common :: decode_b64 (parts_split [2]) ? ; T :: validate_token_message_len (& m_raw) ? ; let is_footer_present = parts_split . len () == 4 ; Ok (Self { message : m_raw , footer : { if is_footer_present { common :: decode_b64 (parts_split [3]) ? } else { Vec :: < u8 > :: new () } } , phantom_t : PhantomData , phantom_v : PhantomData , }) } }
    };
}

impl_130!()