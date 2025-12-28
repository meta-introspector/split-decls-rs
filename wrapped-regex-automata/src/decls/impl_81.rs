macro_rules! deps {
    () => {
        Transition!();
        Epsilons!();
        StateID!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl Transition { const STATE_ID_BITS : u64 = 21 ; const STATE_ID_SHIFT : u64 = 64 - Transition :: STATE_ID_BITS ; const STATE_ID_LIMIT : u64 = 1 << Transition :: STATE_ID_BITS ; const MATCH_WINS_SHIFT : u64 = 64 - (Transition :: STATE_ID_BITS + 1) ; const INFO_MASK : u64 = 0x000003FF_FFFFFFFF ; # [doc = " Return a new transition to the given state ID with the given epsilons."] fn new (match_wins : bool , sid : StateID , epsilons : Epsilons) -> Transition { let match_wins = if match_wins { 1 << Transition :: MATCH_WINS_SHIFT } else { 0 } ; let sid = sid . as_u64 () << Transition :: STATE_ID_SHIFT ; Transition (sid | match_wins | epsilons . 0) } # [doc = " Returns true if and only if this transition points to the DEAD state."] fn is_dead (self) -> bool { self . state_id () == DEAD } # [doc = " Return whether this transition has a \"match wins\" property."] # [doc = ""] # [doc = " When a transition has this property, it means that if a match has been"] # [doc = " found and the search uses leftmost-first semantics, then that match"] # [doc = " should be returned immediately instead of continuing on."] # [doc = ""] # [doc = " The \"match wins\" name comes from RE2, which uses a pretty much"] # [doc = " identical mechanism for implementing leftmost-first semantics."] fn match_wins (& self) -> bool { (self . 0 >> Transition :: MATCH_WINS_SHIFT & 1) == 1 } # [doc = " Return the \"next\" state ID that this transition points to."] fn state_id (& self) -> StateID { StateID :: new_unchecked ((self . 0 >> Transition :: STATE_ID_SHIFT) . as_usize () ,) } # [doc = " Set the \"next\" state ID in this transition."] fn set_state_id (& mut self , sid : StateID) { * self = Transition :: new (self . match_wins () , sid , self . epsilons ()) ; } # [doc = " Return the epsilons embedded in this transition."] fn epsilons (& self) -> Epsilons { Epsilons (self . 0 & Transition :: INFO_MASK) } }
    };
}

impl_81!();