macro_rules! deps {
    () => {
        Error!();
        Signature!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: SignatureMismatch => write ! (f , "Signature doesn't verify") , Error :: WeakPublicKey => write ! (f , "Weak public key") , Error :: InvalidPublicKey => write ! (f , "Invalid public key") , Error :: InvalidSecretKey => write ! (f , "Invalid secret key") , Error :: InvalidSignature => write ! (f , "Invalid signature") , Error :: InvalidSeed => write ! (f , "Invalid seed length") , Error :: InvalidBlind => write ! (f , "Invalid blind length") , Error :: InvalidNoise => write ! (f , "Invalid noise length") , Error :: ParseError => write ! (f , "Parse error") , Error :: NonCanonical => write ! (f , "Non-canonical encoding") , } } }
    };
}

impl_12!()