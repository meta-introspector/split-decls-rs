macro_rules! SiblingLocation {
    () => {
        # [doc = " Determines if a sibling is above or below in the given level of hierarchy"] # [derive (Default , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] # [allow (missing_docs)] pub enum SiblingLocation { Above , Below , AboveAndBelow , # [default] NotFound , }
    };
}

SiblingLocation!()