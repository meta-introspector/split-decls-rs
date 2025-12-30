// Generated macro for get_root_page_table (function)
macro_rules! Depcrate_schedulerget_root_page_table {
() => {
// Module: crate::scheduler
// Provides: {"get_root_page_table"}
// Dependencies: {}
# [cfg (all (target_arch = "x86_64" , feature = "common-os"))] pub (crate) fn get_root_page_table () -> usize { let current_task_borrowed = core_scheduler () . current_task . borrow_mut () ; current_task_borrowed . root_page_table }
};
}
