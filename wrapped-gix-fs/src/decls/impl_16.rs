macro_rules! deps {
    () => {
        SharedFileSnapshotMut!();
        FileSnapshot!();
        SharedFileSnapshot!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T : std :: fmt :: Debug > SharedFileSnapshotMut < T > { # [doc = " Create a new instance of this type."] # [doc = ""] # [doc = " Useful in case `Default::default()` isn't working for some reason."] pub fn new () -> Self { SharedFileSnapshotMut (MutableOnDemand :: new (None)) } # [doc = " Refresh `state` forcefully by re-`open`ing the resource. Note that `open()` returns `None` if the resource isn't"] # [doc = " present on disk, and that it's critical that the modified time is obtained _before_ opening the resource."] pub fn force_refresh < E > (& self , open : impl FnOnce () -> Result < Option < (std :: time :: SystemTime , T) > , E > ,) -> Result < () , E > { let mut state = get_mut (& self . 0) ; * state = open () ? . map (| (modified , value) | OwnShared :: new (FileSnapshot { value , modified })) ; Ok (()) } # [doc = " Assure that the resource in `state` is up-to-date by comparing the `current_modification_time` with the one we know in `state`"] # [doc = " and by acting accordingly."] # [doc = " Returns the potentially updated/reloaded resource if it is still present on disk, which then represents a snapshot that is up-to-date"] # [doc = " in that very moment, or `None` if the underlying file doesn't exist."] # [doc = ""] # [doc = " Note that even though this is racy, each time a request is made there is a chance to see the actual state."] pub fn recent_snapshot < E > (& self , mut current_modification_time : impl FnMut () -> Option < std :: time :: SystemTime > , open : impl FnOnce () -> Result < Option < T > , E > ,) -> Result < Option < SharedFileSnapshot < T > > , E > { let state = get_ref (self) ; let recent_modification = current_modification_time () ; let buffer = match (& * state , recent_modification) { (None , None) => (* state) . clone () , (Some (_) , None) => { drop (state) ; let mut state = get_mut (self) ; * state = None ; (* state) . clone () } (Some (snapshot) , Some (modified_time)) => { if snapshot . modified < modified_time { drop (state) ; let mut state = get_mut (self) ; if let (Some (_snapshot) , Some (modified_time)) = (& * state , current_modification_time ()) { * state = open () ? . map (| value | { OwnShared :: new (FileSnapshot { value , modified : modified_time , }) }) ; } (* state) . clone () } else { Some (snapshot . clone ()) } } (None , Some (_modified_time)) => { drop (state) ; let mut state = get_mut (self) ; if let (None , Some (modified_time)) = (& * state , current_modification_time ()) { * state = open () ? . map (| value | { OwnShared :: new (FileSnapshot { value , modified : modified_time , }) }) ; } (* state) . clone () } } ; Ok (buffer) } }
    };
}

impl_16!()