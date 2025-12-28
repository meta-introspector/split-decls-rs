macro_rules! FetchPrune {
    () => {
        # [doc = " Configuration for how pruning is done on a fetch"] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum FetchPrune { # [doc = " Use the setting from the configuration"] Unspecified , # [doc = " Force pruning on"] On , # [doc = " Force pruning off"] Off , }
    };
}

FetchPrune!();