macro_rules! UNIX_EPOCH_DAY {
    () => {
        # [doc = " Number of days between January 1, 1970 and December 31, 1 BCE which we define to be day 0."] # [doc = " 4 full leap year cycles until December 31, 1600     4 * 146097 = 584388"] # [doc = " 1 day until January 1, 1601                                           1"] # [doc = " 369 years until January 1, 1970                      369 * 365 = 134685"] # [doc = " of which floor(369 / 4) are leap years          floor(369 / 4) =     92"] # [doc = " except for 1700, 1800 and 1900                                       -3 +"] # [doc = "                                                                  --------"] # [doc = "                                                                  719163"] pub (crate) const UNIX_EPOCH_DAY : i64 = 719_163 ;
    };
}

UNIX_EPOCH_DAY!();