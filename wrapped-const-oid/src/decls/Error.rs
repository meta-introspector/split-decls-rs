macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " OID errors."] # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Ord)] pub enum Error { # [doc = " Arc exceeds allowed range (i.e. for first or second OID)"] ArcInvalid { # [doc = " Arc value that is erroneous."] arc : Arc , } , # [doc = " Arc is too big (exceeds 32-bit limits of this library)."] # [doc = ""] # [doc = " Technically the size of an arc is not constrained by X.660, however"] # [doc = " this library has elected to use `u32` as the arc representation as"] # [doc = " sufficient for PKIX/PKCS usages."] ArcTooBig , # [doc = " Base 128 encoding error (used in BER/DER serialization of arcs)."] Base128 , # [doc = " Expected a digit, but was provided something else."] DigitExpected { # [doc = " What was found instead of a digit"] actual : u8 , } , # [doc = " Input data is empty."] Empty , # [doc = " OID length is invalid (too short or too long)."] Length , # [doc = " Arithmetic overflow (or underflow) errors."] # [doc = ""] # [doc = " These generally indicate a bug in the `const-oid` crate."] Overflow , # [doc = " Repeated `..` characters in input data."] RepeatedDot , # [doc = " Trailing `.` character at end of input."] TrailingDot , }
    };
}

Error!()