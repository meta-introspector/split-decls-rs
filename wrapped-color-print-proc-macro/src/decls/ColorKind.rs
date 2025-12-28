macro_rules! ColorKind {
    () => {
        # [doc = " Which \"kind\" of color has to be changed."] # [derive (Debug , PartialEq , Clone)] pub enum ColorKind { Background , Foreground , }
    };
}

ColorKind!()