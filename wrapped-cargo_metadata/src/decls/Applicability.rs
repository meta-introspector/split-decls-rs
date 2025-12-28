macro_rules! Applicability {
    () => {
        # [doc = " Whether a suggestion can be safely applied."] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum Applicability { # [doc = " The suggested replacement can be applied automatically safely"] MachineApplicable , # [doc = " The suggested replacement has placeholders that will need to be manually"] # [doc = " replaced."] HasPlaceholders , # [doc = " The suggested replacement may be incorrect in some circumstances. Needs"] # [doc = " human review."] MaybeIncorrect , # [doc = " The suggested replacement will probably not work."] Unspecified , }
    };
}

Applicability!()