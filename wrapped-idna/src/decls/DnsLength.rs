macro_rules! DnsLength {
    () => {
        # [doc = " The UTS 46 _VerifyDNSLength_ flag."] # [derive (PartialEq , Eq , Copy , Clone)] # [non_exhaustive] pub enum DnsLength { # [doc = " _VerifyDNSLength=false_. (Possibly relevant for allowing non-DNS naming systems.)"] Ignore , # [doc = " _VerifyDNSLength=true_ with the exception that the trailing root label dot is"] # [doc = " allowed."] VerifyAllowRootDot , # [doc = " _VerifyDNSLength=true_. (The trailing root label dot is not allowed.)"] Verify , }
    };
}

DnsLength!();