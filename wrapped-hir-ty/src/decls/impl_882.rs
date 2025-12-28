macro_rules! deps {
    () => {
        BasicBlockId!();
        BasicBlock!();
        SwitchTargets!();
    };
}

macro_rules! impl_882 {
    () => {
        deps!();
        impl < 'db > SwitchTargets < 'db > { # [doc = " Creates switch targets from an iterator of values and target blocks."] # [doc = ""] # [doc = " The iterator may be empty, in which case the `SwitchInt` instruction is equivalent to"] # [doc = " `goto otherwise;`."] pub fn new (targets : impl Iterator < Item = (u128 , BasicBlockId < 'db >) > , otherwise : BasicBlockId < 'db > ,) -> Self { let (values , mut targets) : (SmallVec < _ > , SmallVec < _ >) = targets . unzip () ; targets . push (otherwise) ; Self { values , targets } } # [doc = " Builds a switch targets definition that jumps to `then` if the tested value equals `value`,"] # [doc = " and to `else_` if not."] pub fn static_if (value : u128 , then : BasicBlockId < 'db > , else_ : BasicBlockId < 'db >) -> Self { Self { values : smallvec ! [value] , targets : smallvec ! [then , else_] } } # [doc = " Returns the fallback target that is jumped to when none of the values match the operand."] pub fn otherwise (& self) -> BasicBlockId < 'db > { * self . targets . last () . unwrap () } # [doc = " Returns an iterator over the switch targets."] # [doc = ""] # [doc = " The iterator will yield tuples containing the value and corresponding target to jump to, not"] # [doc = " including the `otherwise` fallback target."] # [doc = ""] # [doc = " Note that this may yield 0 elements. Only the `otherwise` branch is mandatory."] pub fn iter (& self) -> impl Iterator < Item = (u128 , BasicBlockId < 'db >) > + '_ { iter :: zip (& self . values , & self . targets) . map (| (x , y) | (* x , * y)) } # [doc = " Returns a slice with all possible jump targets (including the fallback target)."] pub fn all_targets (& self) -> & [BasicBlockId < 'db >] { & self . targets } # [doc = " Finds the `BasicBlock` to which this `SwitchInt` will branch given the"] # [doc = " specific value. This cannot fail, as it'll return the `otherwise`"] # [doc = " branch if there's not a specific match for the value."] pub fn target_for_value (& self , value : u128) -> BasicBlockId < 'db > { self . iter () . find_map (| (v , t) | (v == value) . then_some (t)) . unwrap_or_else (| | self . otherwise ()) } }
    };
}

impl_882!();