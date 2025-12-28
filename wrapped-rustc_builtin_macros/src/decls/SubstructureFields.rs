macro_rules! deps {
    () => {
        FieldInfo!();
        StaticFields!();
    };
}

macro_rules! SubstructureFields {
    () => {
        deps!();
        # [doc = " A summary of the possible sets of fields."] pub (crate) enum SubstructureFields < 'a > { # [doc = " A non-static method where `Self` is a struct."] Struct (& 'a ast :: VariantData , Vec < FieldInfo >) , # [doc = " A non-static method handling the entire enum at once"] # [doc = " (after it has been determined that none of the enum"] # [doc = " variants has any fields)."] AllFieldlessEnum (& 'a ast :: EnumDef) , # [doc = " Matching variants of the enum: variant index, ast::Variant,"] # [doc = " fields: the field name is only non-`None` in the case of a struct"] # [doc = " variant."] EnumMatching (& 'a ast :: Variant , Vec < FieldInfo >) , # [doc = " The discriminant of an enum. The first field is a `FieldInfo` for the discriminants, as"] # [doc = " if they were fields. The second field is the expression to combine the"] # [doc = " discriminant expression with; it will be `None` if no match is necessary."] EnumDiscr (FieldInfo , Option < Box < Expr > >) , # [doc = " A static method where `Self` is a struct."] StaticStruct (& 'a ast :: VariantData , StaticFields) , # [doc = " A static method where `Self` is an enum."] StaticEnum (& 'a ast :: EnumDef) , }
    };
}

SubstructureFields!();