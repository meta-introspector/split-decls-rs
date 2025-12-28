macro_rules! deps {
    () => {
        OutOfRange!();
    };
}

macro_rules! ParseErrorKind {
    () => {
        deps!();
        # [doc = " The category of parse error"] # [allow (clippy :: manual_non_exhaustive)] # [derive (Debug , Clone , PartialEq , Eq , Copy , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub enum ParseErrorKind { # [doc = " Given field is out of permitted range."] OutOfRange , # [doc = " There is no possible date and time value with given set of fields."] # [doc = ""] # [doc = " This does not include the out-of-range conditions, which are trivially invalid."] # [doc = " It includes the case that there are one or more fields that are inconsistent to each other."] Impossible , # [doc = " Given set of fields is not enough to make a requested date and time value."] # [doc = ""] # [doc = " Note that there *may* be a case that given fields constrain the possible values so much"] # [doc = " that there is a unique possible value. Chrono only tries to be correct for"] # [doc = " most useful sets of fields however, as such constraint solving can be expensive."] NotEnough , # [doc = " The input string has some invalid character sequence for given formatting items."] Invalid , # [doc = " The input string has been prematurely ended."] TooShort , # [doc = " All formatting items have been read but there is a remaining input."] TooLong , # [doc = " There was an error on the formatting string, or there were non-supported formatting items."] BadFormat , # [doc (hidden)] __Nonexhaustive , }
    };
}

ParseErrorKind!()