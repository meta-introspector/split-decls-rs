macro_rules! IceBugReport {
    () => {
        # [derive (Diagnostic)] # [diag (driver_impl_ice_bug_report)] pub (crate) struct IceBugReport < 'a > { pub bug_report_url : & 'a str , }
    };
}

IceBugReport!();