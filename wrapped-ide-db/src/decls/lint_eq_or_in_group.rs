macro_rules! lint_eq_or_in_group {
    () => {
        # [doc = " Checks if the given lint is equal or is contained by the other lint which may or may not be a group."] pub fn lint_eq_or_in_group (lint : & str , lint_is : & str) -> bool { if lint == lint_is { return true ; } if let Some (group) = generated :: lints :: DEFAULT_LINT_GROUPS . iter () . chain (generated :: lints :: CLIPPY_LINT_GROUPS . iter ()) . chain (generated :: lints :: RUSTDOC_LINT_GROUPS . iter ()) . find (| & check | check . lint . label == lint_is) { group . children . contains (& lint) } else { false } }
    };
}

lint_eq_or_in_group!()