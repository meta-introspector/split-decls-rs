macro_rules! TableLayout {
    () => {
        # [doc = " Helper which allows the max calculation for `ctrl_align` to be statically computed for each `T`"] # [doc = " while keeping the rest of `calculate_layout_for` independent of `T`"] # [derive (Copy , Clone)] struct TableLayout { size : usize , ctrl_align : usize , }
    };
}

TableLayout!()