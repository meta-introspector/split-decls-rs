macro_rules! deps {
    () => {
        InvalidIssueStringCause!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl InvalidIssueStringCause { pub (crate) fn from_int_error_kind (span : Span , kind : & IntErrorKind) -> Option < Self > { match kind { IntErrorKind :: Empty => Some (Self :: Empty { span }) , IntErrorKind :: InvalidDigit => Some (Self :: InvalidDigit { span }) , IntErrorKind :: PosOverflow => Some (Self :: PosOverflow { span }) , IntErrorKind :: NegOverflow => Some (Self :: NegOverflow { span }) , IntErrorKind :: Zero => Some (Self :: MustNotBeZero { span }) , _ => None , } } }
    };
}

impl_324!();