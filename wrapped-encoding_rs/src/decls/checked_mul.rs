macro_rules! checked_mul {
    () => {
        # [inline (always)] fn checked_mul (num : usize , opt : Option < usize >) -> Option < usize > { if let Some (n) = opt { n . checked_mul (num) } else { None } }
    };
}

checked_mul!()