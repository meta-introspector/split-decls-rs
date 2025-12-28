macro_rules! deps {
    () => {
        PasswordHash!();
    };
}

macro_rules! impl_637 {
    () => {
        deps!();
        impl core :: fmt :: Debug for PasswordHash { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "PasswordHash {{ encoded_password_hash: [***OMITTED***], password_hash: [***OMITTED***], iterations: \
             {:?}, memory: {:?} }}" , self . iterations , self . memory) } }
    };
}

impl_637!();