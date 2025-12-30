// Generated macro for find_best_applicable_pattern (function)
macro_rules! Depcrate_personnames_specifications_applicable_patternfind_best_applicable_pattern {
() => {
// Module: crate::personnames::specifications::applicable_pattern
// Provides: {"find_best_applicable_pattern"}
// Dependencies: {}
pub fn find_best_applicable_pattern < 'lt > (applicable_pattern : & 'lt [PersonNamePattern < 'lt >] , available_name_fields : & 'lt [& NameField] ,) -> Result < & 'lt PersonNamePattern < 'lt > , PersonNamesFormatterError > { let (_ , _ , max_applicable_pattern) = applicable_pattern . iter () . fold ((0 , u8 :: MAX as usize , None) , | current_max , element | { let (max_used_field_count , max_missing_field_count , _) = & current_max ; let (used_field_count , missing_field_count) = & element . match_info (available_name_fields) ; if used_field_count < max_used_field_count { return current_max ; } if used_field_count > max_used_field_count { return (* used_field_count , * missing_field_count , Some (element)) ; } if max_missing_field_count < missing_field_count { return current_max ; } (* used_field_count , * missing_field_count , Some (element)) }) ; max_applicable_pattern . map (Ok) . unwrap_or (Err (PersonNamesFormatterError :: ParseError (String :: from ("Invalid Person name pattern" ,)))) }
};
}
