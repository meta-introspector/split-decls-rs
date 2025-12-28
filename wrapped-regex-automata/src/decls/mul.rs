macro_rules! deps {
    () => {
        DeserializeError!();
    };
}

macro_rules! mul {
    () => {
        deps!();
        # [doc = " Multiply the given numbers, and on overflow, return an error that includes"] # [doc = " 'what' in the error message."] # [doc = ""] # [doc = " This is useful when doing arithmetic with untrusted data."] pub (crate) fn mul (a : usize , b : usize , what : & 'static str ,) -> Result < usize , DeserializeError > { match a . checked_mul (b) { Some (c) => Ok (c) , None => Err (DeserializeError :: arithmetic_overflow (what)) , } }
    };
}

mul!()