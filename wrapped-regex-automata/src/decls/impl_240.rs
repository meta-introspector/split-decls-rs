macro_rules! deps {
    () => {
        LazyStateID!();
        State!();
        StateSaver!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl StateSaver { # [doc = " Create an empty state saver."] fn none () -> StateSaver { StateSaver :: None } # [doc = " Replace this state saver with an empty saver, and if this saver is a"] # [doc = " request to save a state, return that request."] fn take_to_save (& mut self) -> Option < (LazyStateID , State) > { match core :: mem :: replace (self , StateSaver :: None) { StateSaver :: None | StateSaver :: Saved (_) => None , StateSaver :: ToSave { id , state } => Some ((id , state)) , } } # [doc = " Replace this state saver with an empty saver, and if this saver is a"] # [doc = " saved state (or a request to save a state), return that state's ID."] # [doc = ""] # [doc = " The idea here is that a request to save a state isn't necessarily"] # [doc = " honored because it might not be needed. e.g., Some higher level code"] # [doc = " might request a state to be saved on the off chance that the cache gets"] # [doc = " cleared when a new state is added at a lower level. But if that new"] # [doc = " state is never added, then the cache is never cleared and the state and"] # [doc = " its ID remain unchanged."] fn take_saved (& mut self) -> Option < LazyStateID > { match core :: mem :: replace (self , StateSaver :: None) { StateSaver :: None => None , StateSaver :: Saved (id) | StateSaver :: ToSave { id , .. } => Some (id) , } } }
    };
}

impl_240!();