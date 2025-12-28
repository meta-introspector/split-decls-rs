macro_rules! macro_52 {
    () => {
        pin_project ! { # [doc = " Internal Map future"] # [project = MapProj] # [project_replace = MapProjReplace] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub enum Map < Fut , F > { Incomplete { # [pin] future : Fut , f : F , } , Complete , } }
    };
}

macro_52!();