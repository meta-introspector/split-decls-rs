macro_rules! ArtificialField {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum ArtificialField { ArrayLength , FakeBorrow , }
    };
}

ArtificialField!();