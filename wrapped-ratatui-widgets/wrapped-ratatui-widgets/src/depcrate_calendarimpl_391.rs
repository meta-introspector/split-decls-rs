// Generated macro for impl_391 (impl)
macro_rules! Depcrate_calendarimpl_391 {
() => {
// Module: crate::calendar
// Provides: {"impl_391"}
// Dependencies: {}
impl < DS : DateStyler > Monthly < '_ , DS > { fn render_monthly (& self , area : Rect , buf : & mut Buffer) { let layout = Layout :: vertical ([Constraint :: Length (self . show_month . is_some () . into ()) , Constraint :: Length (self . show_weekday . is_some () . into ()) , Constraint :: Fill (1) ,]) ; let [month_header , days_header , days_area] = layout . areas (area) ; if let Some (style) = self . show_month { Line :: styled (format ! ("{} {}" , self . display_date . month () , self . display_date . year ()) , style ,) . alignment (Alignment :: Center) . render (month_header , buf) ; } if let Some (style) = self . show_weekday { Span :: styled (" Su Mo Tu We Th Fr Sa" , style) . render (days_header , buf) ; } let first_of_month = self . display_date . replace_day (1) . unwrap () ; let offset = Duration :: days (first_of_month . weekday () . number_days_from_sunday () . into ()) ; let mut curr_day = first_of_month - offset ; let mut y = days_area . y ; while curr_day . month () != self . display_date . month () . next () { let mut spans = Vec :: with_capacity (14) ; for i in 0 .. 7 { if i == 0 { spans . push (Span :: styled (" " , Style :: default ())) ; } else { spans . push (Span :: styled (" " , self . default_bg ())) ; } spans . push (self . format_date (curr_day)) ; curr_day += Duration :: DAY ; } if buf . area . height > y { buf . set_line (days_area . x , y , & spans . into () , area . width) ; } y += 1 ; } } }
};
}
