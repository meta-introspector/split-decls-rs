macro_rules! borrow_of_local_data {
    () => {
        # [doc = " Determines if a given borrow is borrowing local data"] # [doc = " This is called for all Yield expressions on movable coroutines"] pub (super) fn borrow_of_local_data (place : Place < '_ >) -> bool { ! place . is_indirect () }
    };
}

borrow_of_local_data!();