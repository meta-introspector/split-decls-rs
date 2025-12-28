macro_rules! has_test_related_attribute {
    () => {
        pub fn has_test_related_attribute (attrs : & hir :: AttrsWithOwner) -> bool { attrs . iter () . any (| attr | { let path = attr . path () ; (| | { Some (path . segments () . first () ? . as_str () . starts_with ("test") || path . segments () . last () ? . as_str () . ends_with ("test") ,) }) () . unwrap_or_default () }) }
    };
}

has_test_related_attribute!()