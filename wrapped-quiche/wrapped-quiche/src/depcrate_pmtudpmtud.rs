// Generated macro for Pmtud (struct)
macro_rules! Depcrate_pmtudPmtud {
() => {
// Module: crate::pmtud
// Provides: {"Pmtud"}
// Dependencies: {}
# [derive (Default)] pub struct Pmtud { # [doc = " The PMTU after the completion of PMTUD."] # [doc = " Will be [`None`] if the PMTU is less than the minimum supported MTU."] pmtu : Option < usize > , # [doc = " The current PMTUD probe size. Set to maximum_supported_mtu at"] # [doc = " initialization."] probe_size : usize , # [doc = " The maximum supported MTU."] maximum_supported_mtu : usize , # [doc = " The size of the smallest failed probe."] smallest_failed_probe_size : Option < usize > , # [doc = " The size of the largest successful probe."] largest_successful_probe_size : Option < usize > , # [doc = " Indicates if a PMTUD probe is in flight. Used to limit probes to 1/RTT."] in_flight : bool , }
};
}
