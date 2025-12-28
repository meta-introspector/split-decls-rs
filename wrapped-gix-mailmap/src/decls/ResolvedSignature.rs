macro_rules! ResolvedSignature {
    () => {
        # [doc = " A resolved signature with borrowed fields for a mapped `name` and/or `email`."] pub struct ResolvedSignature < 'a > { # [doc = " The mapped name."] pub name : Option < & 'a BStr > , # [doc = " The mapped email."] pub email : Option < & 'a BStr > , }
    };
}

ResolvedSignature!()