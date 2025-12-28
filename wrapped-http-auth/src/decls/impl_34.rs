macro_rules! deps {
    () => {
        Error!();
        ChallengeRef!();
        DigestClient!();
        Algorithm!();
        Qop!();
        QopSet!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl TryFrom < & ChallengeRef < '_ > > for DigestClient { type Error = String ; fn try_from (value : & ChallengeRef < '_ >) -> Result < Self , Self :: Error > { if ! value . scheme . eq_ignore_ascii_case ("Digest") { return Err (format ! ("DigestClientContext doesn't support challenge scheme {:?}" , value . scheme)) ; } let mut buf_len = 0 ; let mut unused_len = 0 ; let mut realm = None ; let mut domain = None ; let mut nonce = None ; let mut opaque = None ; let mut stale = false ; let mut algorithm_and_session = None ; let mut qop_str = None ; let mut userhash_str = None ; for (k , v) in & value . params { if store_param (k , v , "realm" , & mut realm , & mut buf_len) ? || store_param (k , v , "domain" , & mut domain , & mut buf_len) ? || store_param (k , v , "nonce" , & mut nonce , & mut buf_len) ? || store_param (k , v , "opaque" , & mut opaque , & mut buf_len) ? || store_param (k , v , "qop" , & mut qop_str , & mut unused_len) ? || store_param (k , v , "userhash" , & mut userhash_str , & mut unused_len) ? { } else if k . eq_ignore_ascii_case ("stale") { stale = v . escaped . eq_ignore_ascii_case ("true") ; } else if k . eq_ignore_ascii_case ("algorithm") { algorithm_and_session = Some (Algorithm :: parse (v . escaped) ?) ; } } let realm = realm . ok_or ("missing required parameter realm") ? ; let nonce = nonce . ok_or ("missing required parameter nonce") ? ; if buf_len > u16 :: MAX as usize { return Err (format ! ("Unescaped parameters' length {} exceeds u16::MAX!" , buf_len)) ; } let algorithm_and_session = algorithm_and_session . unwrap_or ((Algorithm :: Md5 , false)) ; let mut buf = String :: with_capacity (buf_len) ; let mut qop = QopSet (0) ; let rfc2069_compat = if let Some (qop_str) = qop_str { let qop_str = qop_str . unescaped_with_scratch (& mut buf) ; for v in qop_str . split (',') { let v = v . trim () ; if v . eq_ignore_ascii_case ("auth") { qop . 0 |= Qop :: Auth as u8 ; } else if v . eq_ignore_ascii_case ("auth-int") { qop . 0 |= Qop :: AuthInt as u8 ; } } if qop . 0 == 0 { return Err (format ! ("no supported qop in {:?}" , qop_str)) ; } buf . clear () ; false } else { qop . 0 |= Qop :: Auth as u8 ; true } ; let userhash ; if let Some (userhash_str) = userhash_str { let userhash_str = userhash_str . unescaped_with_scratch (& mut buf) ; userhash = userhash_str . eq_ignore_ascii_case ("true") ; buf . clear () ; } else { userhash = false ; } ; realm . append_unescaped (& mut buf) ; let domain_start = buf . len () ; if let Some (d) = domain { d . append_unescaped (& mut buf) ; } let opaque_start = buf . len () ; if let Some (o) = opaque { o . append_unescaped (& mut buf) ; } let nonce_start = buf . len () ; nonce . append_unescaped (& mut buf) ; Ok (DigestClient { buf : buf . into_boxed_str () , domain_start : domain_start as u16 , opaque_start : opaque_start as u16 , nonce_start : nonce_start as u16 , algorithm : algorithm_and_session . 0 , session : algorithm_and_session . 1 , stale , rfc2069_compat , userhash , qop , nc : 0 , }) } }
    };
}

impl_34!()