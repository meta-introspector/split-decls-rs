macro_rules! deps {
    () => {
        TypeVariableData!();
        TyVidSubKey!();
        TyVidEqKey!();
    };
}

macro_rules! TypeVariableStorage {
    () => {
        deps!();
        # [derive (Clone , Default)] pub (crate) struct TypeVariableStorage < 'tcx > { # [doc = " The origins of each type variable."] values : IndexVec < TyVid , TypeVariableData > , # [doc = " Two variables are unified in `eq_relations` when we have a"] # [doc = " constraint `?X == ?Y`. This table also stores, for each key,"] # [doc = " the known value."] eq_relations : ut :: UnificationTableStorage < TyVidEqKey < 'tcx > > , # [doc = " Only used by `-Znext-solver` and for diagnostics. Tracks whether"] # [doc = " type variables are related via subtyping at all, ignoring which of"] # [doc = " the two is the subtype."] # [doc = ""] # [doc = " When reporting ambiguity errors, we sometimes want to"] # [doc = " treat all inference vars which are subtypes of each"] # [doc = " others as if they are equal. For this case we compute"] # [doc = " the transitive closure of our subtype obligations here."] # [doc = ""] # [doc = " E.g. when encountering ambiguity errors, we want to suggest"] # [doc = " specifying some method argument or to add a type annotation"] # [doc = " to a local variable. Because subtyping cannot change the"] # [doc = " shape of a type, it's fine if the cause of the ambiguity error"] # [doc = " is only related to the suggested variable via subtyping."] # [doc = ""] # [doc = " Even for something like `let x = returns_arg(); x.method();` the"] # [doc = " type of `x` is only a supertype of the argument of `returns_arg`. We"] # [doc = " still want to suggest specifying the type of the argument."] sub_unification_table : ut :: UnificationTableStorage < TyVidSubKey > , }
    };
}

TypeVariableStorage!();