macro_rules! find_assignments {
    () => {
        # [doc = " Finds all statements that assign directly to local (i.e., X = ...) and returns their"] # [doc = " locations."] fn find_assignments (body : & Body < '_ > , local : Local) -> Vec < Location > { use rustc_middle :: mir :: visit :: Visitor ; struct FindLocalAssignmentVisitor { needle : Local , locations : Vec < Location > , } impl < 'tcx > Visitor < 'tcx > for FindLocalAssignmentVisitor { fn visit_local (& mut self , local : Local , place_context : PlaceContext , location : Location) { if self . needle != local { return ; } if place_context . is_place_assignment () { self . locations . push (location) ; } } } let mut visitor = FindLocalAssignmentVisitor { needle : local , locations : vec ! [] } ; visitor . visit_body (body) ; visitor . locations }
    };
}

find_assignments!()