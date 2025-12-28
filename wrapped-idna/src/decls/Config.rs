macro_rules! Config {
    () => {
        # [doc = " Deprecated configuration API."] # [derive (Clone , Copy)] # [must_use] # [deprecated] pub struct Config { use_std3_ascii_rules : bool , transitional_processing : bool , verify_dns_length : bool , check_hyphens : bool , }
    };
}

Config!();