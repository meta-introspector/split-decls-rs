macro_rules! WalkerIter {
    () => {
        # [doc = " A walker and its context wrapped into an iterator."] # [derive (Clone , Debug)] pub struct WalkerIter < W , C > { walker : W , context : C , }
    };
}

WalkerIter!();