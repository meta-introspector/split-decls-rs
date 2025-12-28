macro_rules! deps {
    () => {
        Local!();
        UntrustedToken!();
        Public!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < T : Purpose < V > , V : Version > UntrustedToken < T , V > { # [doc = " Return untrusted message of this [`UntrustedToken`]."] # [doc = " If it is a [`Local`] token, this is the encrypted message with nonce and tag."] # [doc = " If it is a [`Public`] token, the signature is included."] pub fn untrusted_message (& self) -> & [u8] { & self . message } # [doc = " Return untrusted payload only of this [`UntrustedToken`]'s message body."] # [doc = " If it is a [`Local`] token, this is the encrypted message sans nonce and tag."] # [doc = " If it is a [`Public`] token, the signature is not included."] pub fn untrusted_payload (& self) -> & [u8] { T :: parse_raw_payload (self . untrusted_message ()) } # [doc = " Return untrusted footer of this [`UntrustedToken`]."] # [doc = " Empty if there was no footer in the token."] pub fn untrusted_footer (& self) -> & [u8] { & self . footer } }
    };
}

impl_132!()