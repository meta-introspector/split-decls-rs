macro_rules! AlreadyAsciiLabel {
    () => {
        # [derive (Debug , Clone , Copy)] enum AlreadyAsciiLabel < 'a > { MixedCaseAscii (& 'a [u8]) , MixedCasePunycode (& 'a [u8]) , Other , }
    };
}

AlreadyAsciiLabel!()