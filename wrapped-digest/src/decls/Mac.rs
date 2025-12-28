macro_rules! deps {
    () => {
        MacError!();
        Update!();
        FixedOutput!();
        CtOutput!();
        FixedOutputReset!();
        MacMarker!();
    };
}

macro_rules! Mac {
    () => {
        deps!();
        # [doc = " Convenience wrapper trait covering functionality of Message Authentication algorithms."] # [doc = ""] # [doc = " This trait wraps [`Update`], [`FixedOutput`], and [`MacMarker`] traits"] # [doc = " and provides additional convenience methods."] pub trait Mac : OutputSizeUser + Sized { # [doc = " Update state using the provided data."] fn update (& mut self , data : & [u8]) ; # [doc = " Process input data in a chained manner."] # [must_use] fn chain_update (self , data : impl AsRef < [u8] >) -> Self ; # [doc = " Obtain the result of a [`Mac`] computation as a [`CtOutput`] and consume"] # [doc = " [`Mac`] instance."] fn finalize (self) -> CtOutput < Self > ; # [doc = " Obtain the result of a [`Mac`] computation as a [`CtOutput`] and reset"] # [doc = " [`Mac`] instance."] fn finalize_reset (& mut self) -> CtOutput < Self > where Self : FixedOutputReset ; # [doc = " Reset MAC instance to its initial state."] fn reset (& mut self) where Self : Reset ; # [doc = " Check if tag/code value is correct for the processed input."] fn verify (self , tag : & Output < Self >) -> Result < () , MacError > ; # [doc = " Check if tag/code value is correct for the processed input and reset"] # [doc = " [`Mac`] instance."] fn verify_reset (& mut self , tag : & Output < Self >) -> Result < () , MacError > where Self : FixedOutputReset ; # [doc = " Check truncated tag correctness using all bytes"] # [doc = " of calculated tag."] # [doc = ""] # [doc = " Returns `Error` if `tag` is not valid or not equal in length"] # [doc = " to MAC's output."] fn verify_slice (self , tag : & [u8]) -> Result < () , MacError > ; # [doc = " Check truncated tag correctness using all bytes"] # [doc = " of calculated tag and reset [`Mac`] instance."] # [doc = ""] # [doc = " Returns `Error` if `tag` is not valid or not equal in length"] # [doc = " to MAC's output."] fn verify_slice_reset (& mut self , tag : & [u8]) -> Result < () , MacError > where Self : FixedOutputReset ; # [doc = " Check truncated tag correctness using left side bytes"] # [doc = " (i.e. `tag[..n]`) of calculated tag."] # [doc = ""] # [doc = " Returns `Error` if `tag` is not valid or empty."] fn verify_truncated_left (self , tag : & [u8]) -> Result < () , MacError > ; # [doc = " Check truncated tag correctness using right side bytes"] # [doc = " (i.e. `tag[n..]`) of calculated tag."] # [doc = ""] # [doc = " Returns `Error` if `tag` is not valid or empty."] fn verify_truncated_right (self , tag : & [u8]) -> Result < () , MacError > ; }
    };
}

Mac!()