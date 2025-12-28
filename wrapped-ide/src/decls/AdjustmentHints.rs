macro_rules! AdjustmentHints {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub enum AdjustmentHints { Always , BorrowsOnly , Never , }
    };
}

AdjustmentHints!();