// Generated macro for print_free_disk_space (function)
macro_rules! Depcrate_utilsprint_free_disk_space {
() => {
// Module: crate::utils
// Provides: {"print_free_disk_space"}
// Dependencies: {}
pub fn print_free_disk_space () -> anyhow :: Result < () > { let disks = Disks :: new_with_refreshed_list () ; let available_space : u64 = disks . list () . iter () . map (| d | d . available_space ()) . sum () ; let total_space : u64 = disks . list () . iter () . map (| d | d . total_space ()) . sum () ; let used_space = total_space - available_space ; log :: info ! ("Free disk space: {} out of total {} ({:.2}% used)" , humansize :: format_size (available_space , BINARY) , humansize :: format_size (total_space , BINARY) , (used_space as f64 / total_space as f64) * 100.0) ; Ok (()) }
};
}
