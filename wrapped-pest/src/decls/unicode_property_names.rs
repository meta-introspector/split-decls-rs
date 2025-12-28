macro_rules! unicode_property_names {
    () => {
        # [doc = " Return all available unicode property names"] pub fn unicode_property_names () -> Box < dyn Iterator < Item = & 'static str > > { Box :: new (BINARY_PROPERTY_NAMES . iter () . map (| name | * name) . chain (CATEGORY_PROPERTY_NAMES . iter () . map (| name | * name)) . chain (SCRIPT_PROPERTY_NAMES . iter () . map (| name | * name)) ,) }
    };
}

unicode_property_names!();