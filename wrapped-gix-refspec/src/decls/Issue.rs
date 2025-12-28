macro_rules! deps {
    () => {
        Source!();
    };
}

macro_rules! Issue {
    () => {
        deps!();
        # [doc = " All possible issues found while validating matched mappings."] # [derive (Debug , PartialEq , Eq)] pub enum Issue { # [doc = " Multiple sources try to write the same destination."] # [doc = ""] # [doc = " Note that this issue doesn't take into consideration that these sources might contain the same object behind a reference."] Conflict { # [doc = " The unenforced full name of the reference to be written."] destination_full_ref_name : BString , # [doc = " The list of sources that map to this destination."] sources : Vec < Source > , # [doc = " The list of specs that caused the mapping conflict, each matching the respective one in `sources` to allow both"] # [doc = " `sources` and `specs` to be zipped together."] specs : Vec < BString > , } , }
    };
}

Issue!();