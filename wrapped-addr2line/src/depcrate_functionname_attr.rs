// Generated macro for name_attr (function)
macro_rules! Depcrate_functionname_attr {
() => {
// Module: crate::function
// Provides: {"name_attr"}
// Dependencies: {}
fn name_attr < R > (attr : gimli :: AttributeValue < R > , mut file : DebugFile , unit : gimli :: UnitRef < R > , ctx : & Context < R > , recursion_limit : usize ,) -> Result < Option < R > , Error > where R : gimli :: Reader , { if recursion_limit == 0 { return Ok (None) ; } match attr { gimli :: AttributeValue :: UnitRef (offset) => { name_entry (file , unit , offset , ctx , recursion_limit) } gimli :: AttributeValue :: DebugInfoRef (dr) => { let sections = unit . dwarf ; let (unit , offset) = ctx . find_unit (dr , file) ? ; let unit = gimli :: UnitRef :: new (sections , unit) ; name_entry (file , unit , offset , ctx , recursion_limit) } gimli :: AttributeValue :: DebugInfoRefSup (dr) => { if let Some (sup_sections) = unit . dwarf . sup . as_ref () { file = DebugFile :: Supplementary ; let (unit , offset) = ctx . find_unit (dr , file) ? ; let unit = gimli :: UnitRef :: new (sup_sections , unit) ; name_entry (file , unit , offset , ctx , recursion_limit) } else { Ok (None) } } _ => Ok (None) , } }
};
}
